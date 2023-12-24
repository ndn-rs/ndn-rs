use super::*;

/// NfdVersion: NFD version; this is usually the same as the output of nfd --version, but the forwarder MAY hide its version and return an empty string
/// StartTimestamp: timestamp (milliseconds since UNIX epoch) when the forwarder started
/// CurrentTimestamp: timestamp (milliseconds since UNIX epoch) of current time
/// NNameTreeEntries: number of NameTree entries
/// NFibEntries: number of FIB entries
/// NPitEntries: number of PIT entries
/// NMeasurementsEntries: number of Measurements entries
/// NCsEntries: number of CS entries
/// NInInterests: number of incoming Interest packets processed since the forwarder started
/// NInData: number of incoming Data packets processed since the forwarder started
/// NInNacks: number of incoming Nack packets processed since the forwarder started
/// NOutInterests: number of outgoing Interest packets processed since the forwarder started
/// NOutData: number of outgoing Data packets processed since the forwarder started
/// NOutNacks: number of outgoing Nack packets processed since the forwarder started
/// NSatisfiedInterests: number of satisfied Interests, incremented when a PIT entry that has been satisfied is being removed
/// NUnsatisfiedInterests: number of unsatisfied Interests, incremented when a PIT entry that has not been satisfied is being removed

#[derive(Debug)]
pub struct GeneralStatus {
    pub version: NfdVersion,
    pub start_timestamp: StartTimestamp,
    pub current_timestamp: CurrentTimestamp,
    pub n_name_tree_entries: NNameTreeEntries,
    pub n_fib_entries: NFibEntries,
    pub n_pit_entries: NPitEntries,
    pub n_measurements_entries: NMeasurementsEntries,
    pub n_cs_entries: NCsEntries,
    pub n_in_interests: NInInterests,
    pub n_in_data: NInData,
    pub n_in_nacks: NInNacks,
    pub n_out_interests: NOutInterests,
    pub n_out_data: NOutData,
    pub n_out_nacks: NOutNacks,
    pub n_satisfied_interests: NSatisfiedInterests,
    pub n_unsatisfied_interests: NUnsatisfiedInterests,
}

impl GeneralStatus {
    const NAME: &'static str = "/localhost/nfd/status/general";
}

impl TryFrom<tlv::Data> for GeneralStatus {
    type Error = tlv::DecodeError;

    fn try_from(data: tlv::Data) -> Result<Self, Self::Error> {
        let mut items = data
            .name_starts_with(Self::NAME)?
            .content
            .ok_or_else(|| tlv::DecodeError::invalid("Invalid content of Data Packet"))?
            .into_iter();

        let version = items
            .next()
            .invalid_data("NfdVersion must be present")?
            .try_into()?;

        let start_timestamp = items
            .next()
            .invalid_data("StartTimestamp must be present")?
            .try_into()?;

        let current_timestamp = items
            .next()
            .invalid_data("CurrentTimestamp must be present")?
            .try_into()?;

        let n_name_tree_entries = items
            .next()
            .invalid_data("NNameTreeEntries must be present")?
            .try_into()?;

        let n_fib_entries = items
            .next()
            .invalid_data("NFibEntries must be present")?
            .try_into()?;

        let n_pit_entries = items
            .next()
            .invalid_data("NPitEntries must be present")?
            .try_into()?;

        let n_measurements_entries = items
            .next()
            .invalid_data("NMeasurementEntries must be present")?
            .try_into()?;

        let n_cs_entries = items
            .next()
            .invalid_data("NCsEntries must be present")?
            .try_into()?;

        let n_in_interests = items
            .next()
            .invalid_data("NInInterests must be present")?
            .try_into()?;

        let n_in_data = items
            .next()
            .invalid_data("NInData must be present")?
            .try_into()?;

        let n_in_nacks = items
            .next()
            .invalid_data("NInNacks must be present")?
            .try_into()?;

        let n_out_interests = items
            .next()
            .invalid_data("NOutInterests must be present")?
            .try_into()?;

        let n_out_data = items
            .next()
            .invalid_data("NOutData must be present")?
            .try_into()?;

        let n_out_nacks = items
            .next()
            .invalid_data("NOutNacks must be present")?
            .try_into()?;

        let n_satisfied_interests = items
            .next()
            .invalid_data("NSatisfiedInterests must be present")?
            .try_into()?;

        let n_unsatisfied_interests = items
            .next()
            .invalid_data("NUnsatisfiedInterests must be present")?
            .try_into()?;

        // let
        Ok(Self {
            version,
            start_timestamp,
            current_timestamp,
            n_name_tree_entries,
            n_fib_entries,
            n_pit_entries,
            n_measurements_entries,
            n_cs_entries,
            n_in_interests,
            n_in_data,
            n_in_nacks,
            n_out_interests,
            n_out_data,
            n_out_nacks,
            n_satisfied_interests,
            n_unsatisfied_interests,
        })
    }
}

tlv::utf8_string!(NfdVersion => tlv::Type::NfdVersion);
tlv::milliseconds!(StartTimestamp => tlv::Type::StartTimestamp);
tlv::milliseconds!(CurrentTimestamp => tlv::Type::CurrentTimestamp);
tlv::non_negative_number!(NNameTreeEntries => tlv::Type::NNameTreeEntries); // number of NameTree entries
tlv::non_negative_number!(NFibEntries => tlv::Type::NFibEntries); // number of FIB entries
tlv::non_negative_number!(NPitEntries => tlv::Type::NPitEntries); // number of PIT entries
tlv::non_negative_number!(NMeasurementsEntries => tlv::Type::NMeasurementsEntries); // number of Measurements entries
tlv::non_negative_number!(NCsEntries => tlv::Type::NCsEntries); // number of CS entries
tlv::non_negative_number!(NInInterests => tlv::Type::NInInterests); // number of incoming Interest packets processed since the forwarder started
tlv::non_negative_number!(NInData => tlv::Type::NInData); // number of incoming Data packets processed since the forwarder started
tlv::non_negative_number!(NInNacks => tlv::Type::NInNacks); // number of incoming Nack packets processed since the forwarder started
tlv::non_negative_number!(NOutInterests => tlv::Type::NOutInterests); // number of outgoing Interest packets processed since the forwarder started
tlv::non_negative_number!(NOutData => tlv::Type::NOutData); // number of outgoing Data packets processed since the forwarder started
tlv::non_negative_number!(NOutNacks => tlv::Type::NOutNacks); // number of outgoing Nack packets processed since the forwarder started
tlv::non_negative_number!(NSatisfiedInterests => tlv::Type::NSatisfiedInterests); // number of satisfied Interests, incremented when a PIT entry that has been satisfied is being removed
tlv::non_negative_number!(NUnsatisfiedInterests => tlv::Type::NUnsatisfiedInterests); // number of unsatisfied Interests, incremented when a PIT entry that has not been satisfied is being removed
