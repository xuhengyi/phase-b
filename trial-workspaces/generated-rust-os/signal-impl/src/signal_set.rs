use signal::SignalNo;

/// Bitset helper that tracks pending signals.
///
/// Internally this stores up to 64 signal bits, mapping directly to the
/// `SignalNo` discriminants. `SignalNo::ERR` is never recorded.
#[derive(Clone, Copy, Debug, Default)]
pub struct SignalSet {
    bits: u64,
}

impl SignalSet {
    #[inline]
    const fn bit_of(signal: SignalNo) -> Option<u64> {
        match signal {
            SignalNo::ERR => None,
            _ => Some(1u64 << (signal as u8)),
        }
    }

    pub fn add(&mut self, signal: SignalNo) {
        if let Some(bit) = Self::bit_of(signal) {
            self.bits |= bit;
        }
    }

    pub fn remove(&mut self, signal: SignalNo) -> bool {
        if let Some(bit) = Self::bit_of(signal) {
            let present = self.bits & bit != 0;
            self.bits &= !bit;
            return present;
        }
        false
    }

    pub fn contains(&self, signal: SignalNo) -> bool {
        Self::bit_of(signal)
            .map(|bit| self.bits & bit != 0)
            .unwrap_or(false)
    }

    pub fn clear(&mut self) {
        self.bits = 0;
    }
}
