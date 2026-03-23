from __future__ import annotations

from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any
import json
import os
import urllib.error
import urllib.request

from .config import APIConfig


@dataclass(slots=True)
class FileEdit:
    path: str
    content: str


@dataclass(slots=True)
class EditProposal:
    summary: str
    files: list[FileEdit]
    backtrack_crates: list[str]
    notes: str


class ModelAPIError(RuntimeError):
    pass


class ModelClient:
    def __init__(self, config: APIConfig) -> None:
        self.config = config
        api_key = os.environ.get(config.api_key_env)
        if not api_key:
            raise ModelAPIError(f"missing API key environment variable: {config.api_key_env}")
        self.api_key = api_key

    def generate_edit_proposal(self, system_prompt: str, user_prompt: str) -> EditProposal:
        payload = self._request_payload(system_prompt, user_prompt)
        raw_text = self._post_json(payload)
        try:
            raw = json.loads(raw_text)
        except json.JSONDecodeError as exc:
            raise ModelAPIError(f"model did not return valid JSON: {exc}") from exc

        files = [FileEdit(path=item["path"], content=item["content"]) for item in raw.get("files", [])]
        return EditProposal(
            summary=str(raw.get("summary", "")),
            files=files,
            backtrack_crates=[str(item) for item in raw.get("backtrack_crates", [])],
            notes=str(raw.get("notes", "")),
        )

    def _request_payload(self, system_prompt: str, user_prompt: str) -> tuple[str, dict[str, Any]]:
        if self.config.endpoint == "responses":
            url = f"{self.config.base_url}/responses"
            payload: dict[str, Any] = {
                "model": self.config.model,
                "input": [
                    {
                        "role": "system",
                        "content": [{"type": "input_text", "text": system_prompt}],
                    },
                    {
                        "role": "user",
                        "content": [{"type": "input_text", "text": user_prompt}],
                    },
                ],
                "temperature": self.config.temperature,
                "text": {"format": {"type": "json_object"}},
            }
            return url, payload

        url = f"{self.config.base_url}/chat/completions"
        payload = {
            "model": self.config.model,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_prompt},
            ],
            "temperature": self.config.temperature,
            "response_format": {"type": "json_object"},
        }
        return url, payload

    def _post_json(self, request_payload: tuple[str, dict[str, Any]]) -> str:
        url, payload = request_payload
        body = json.dumps(payload).encode("utf-8")
        request = urllib.request.Request(
            url,
            data=body,
            headers={
                "Authorization": f"Bearer {self.api_key}",
                "Content-Type": "application/json",
            },
            method="POST",
        )
        try:
            with urllib.request.urlopen(request, timeout=self.config.timeout_seconds) as response:
                raw = response.read().decode("utf-8")
        except urllib.error.HTTPError as exc:
            detail = exc.read().decode("utf-8", errors="replace")
            raise ModelAPIError(f"API request failed with {exc.code}: {detail}") from exc
        except urllib.error.URLError as exc:
            raise ModelAPIError(f"API request failed: {exc}") from exc

        data = json.loads(raw)
        if self.config.endpoint == "responses":
            if data.get("output_text"):
                return str(data["output_text"])
            for item in data.get("output", []):
                for content in item.get("content", []):
                    text = content.get("text")
                    if text:
                        return str(text)
            raise ModelAPIError(f"responses output did not contain text: {raw}")

        try:
            content = data["choices"][0]["message"]["content"]
        except (KeyError, IndexError, TypeError) as exc:
            raise ModelAPIError(f"chat completions output was malformed: {raw}") from exc

        if isinstance(content, list):
            parts = []
            for block in content:
                text = block.get("text")
                if text:
                    parts.append(text)
            return "\n".join(parts)
        return str(content)


def proposal_to_jsonable(proposal: EditProposal) -> dict[str, Any]:
    return {
        "summary": proposal.summary,
        "files": [asdict(item) for item in proposal.files],
        "backtrack_crates": proposal.backtrack_crates,
        "notes": proposal.notes,
    }
