/// An error when parsing a timeframe.
#[derive(Debug, Clone, Copy)]
pub enum TimeframeError {
    /// An invalid timeframe was specified.
    InvalidTimeframe,
}

/// A representation of a timeframe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Timeframe {
    /// Every day.
    Daily,
    /// Every week.
    Weekly,
    /// Every other week.
    Biweekly,
    /// Every month.
    Monthly,
    /// Every other month.
    Bimonthly,
    /// Every three months.
    Quarterly,
    /// Every six months.
    Semiannually,
    /// Every year.
    Annually,
}

impl Timeframe {
    /// Gets the timeframe from its internal name in the database.
    pub fn from_internal_name(timeframe: &str) -> Result<Self, TimeframeError> {
        match timeframe {
            "DAILY" => Ok(Self::Daily),
            "WEEKLY" => Ok(Self::Weekly),
            "BIWEEKLY" => Ok(Self::Biweekly),
            "MONTHLY" => Ok(Self::Monthly),
            "BIMONTHLY" => Ok(Self::Bimonthly),
            "QUARTERLY" => Ok(Self::Quarterly),
            "SEMIANNUALLY" => Ok(Self::Semiannually),
            "ANNUALLY" => Ok(Self::Annually),
            _ => Err(TimeframeError::InvalidTimeframe),
        }
    }

    /// Gets the internal name of the timeframe.
    pub fn to_internal_name(&self) -> String {
        match self {
            Self::Daily => "DAILY",
            Self::Weekly => "WEEKLY",
            Self::Biweekly => "BIWEEKLY",
            Self::Monthly => "MONTHLY",
            Self::Bimonthly => "BIMONTHLY",
            Self::Quarterly => "QUARTERLY",
            Self::Semiannually => "SEMIANNUALLY",
            Self::Annually => "ANNUALLY",
        }
        .to_owned()
    }

    /// Gets the human-readable string representation of the timeframe.
    pub fn as_str(&self) -> String {
        match self {
            Self::Daily => "Daily",
            Self::Weekly => "Weekly",
            Self::Biweekly => "Biweekly",
            Self::Monthly => "Monthly",
            Self::Bimonthly => "Bimonthly",
            Self::Quarterly => "Quarterly",
            Self::Semiannually => "Semiannually",
            Self::Annually => "Annually",
        }
        .to_owned()
    }
}

/// Timeframe tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeframe() {
        // Parse timeframes
        let daily = Timeframe::from_internal_name("DAILY").unwrap();
        assert_eq!(daily, Timeframe::Daily);
        let weekly = Timeframe::from_internal_name("WEEKLY").unwrap();
        assert_eq!(weekly, Timeframe::Weekly);
        let biweekly = Timeframe::from_internal_name("BIWEEKLY").unwrap();
        assert_eq!(biweekly, Timeframe::Biweekly);
        let monthly = Timeframe::from_internal_name("MONTHLY").unwrap();
        assert_eq!(monthly, Timeframe::Monthly);
        let bimonthly = Timeframe::from_internal_name("BIMONTHLY").unwrap();
        assert_eq!(bimonthly, Timeframe::Bimonthly);
        let quarterly = Timeframe::from_internal_name("QUARTERLY").unwrap();
        assert_eq!(quarterly, Timeframe::Quarterly);
        let semiannually = Timeframe::from_internal_name("SEMIANNUALLY").unwrap();
        assert_eq!(semiannually, Timeframe::Semiannually);
        let annually = Timeframe::from_internal_name("ANNUALLY").unwrap();
        assert_eq!(annually, Timeframe::Annually);
        Timeframe::from_internal_name("TIMEFRAME").unwrap_err();
        Timeframe::from_internal_name("HOURLY").unwrap_err();

        // Get timeframe string names
        assert_eq!(&daily.to_internal_name(), "DAILY");
        assert_eq!(&weekly.to_internal_name(), "WEEKLY");
        assert_eq!(&biweekly.to_internal_name(), "BIWEEKLY");
        assert_eq!(&monthly.to_internal_name(), "MONTHLY");
        assert_eq!(&bimonthly.to_internal_name(), "BIMONTHLY");
        assert_eq!(&quarterly.to_internal_name(), "QUARTERLY");
        assert_eq!(&semiannually.to_internal_name(), "SEMIANNUALLY");
        assert_eq!(&annually.to_internal_name(), "ANNUALLY");

        // Get timeframe human-readable names
        assert_eq!(&daily.as_str(), "Daily");
        assert_eq!(&weekly.as_str(), "Weekly");
        assert_eq!(&biweekly.as_str(), "Biweekly");
        assert_eq!(&monthly.as_str(), "Monthly");
        assert_eq!(&bimonthly.as_str(), "Bimonthly");
        assert_eq!(&quarterly.as_str(), "Quarterly");
        assert_eq!(&semiannually.as_str(), "Semiannually");
        assert_eq!(&annually.as_str(), "Annually");
    }
}
