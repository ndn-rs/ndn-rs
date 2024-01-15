use super::*;

pub use channels::ChannelStatus;

mod channels;

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
    pub n_in_interests: face::NInInterests,
    pub n_in_data: face::NInData,
    pub n_in_nacks: face::NInNacks,
    pub n_out_interests: face::NOutInterests,
    pub n_out_data: face::NOutData,
    pub n_out_nacks: face::NOutNacks,
    pub n_satisfied_interests: NSatisfiedInterests,
    pub n_unsatisfied_interests: NUnsatisfiedInterests,
}

impl GeneralStatus {
    pub const NAME: &'static str = "/localhost/nfd/status/general";
}

impl tlv::core::TlvCodec for GeneralStatus {
    type Error = tlv::DecodeError;
    const TYPE: tlv::Type = tlv::Type::Unassigned;

    fn total_size(&self) -> usize {
        // use tlv::core::TlvCodec;
        [
            self.version.total_size(),
            self.start_timestamp.total_size(),
            self.current_timestamp.total_size(),
            self.n_name_tree_entries.total_size(),
            self.n_fib_entries.total_size(),
            self.n_pit_entries.total_size(),
            self.n_measurements_entries.total_size(),
            self.n_cs_entries.total_size(),
            self.n_in_interests.total_size(),
            self.n_in_data.total_size(),
            self.n_in_nacks.total_size(),
            self.n_out_interests.total_size(),
            self.n_out_data.total_size(),
            self.n_out_nacks.total_size(),
            self.n_satisfied_interests.total_size(),
            self.n_unsatisfied_interests.total_size(),
        ]
        .into_iter()
        .sum()
    }

    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        self.version.encode(dst)?;
        self.start_timestamp.encode(dst)?;
        self.current_timestamp.encode(dst)?;
        self.n_name_tree_entries.encode(dst)?;
        self.n_fib_entries.encode(dst)?;
        self.n_pit_entries.encode(dst)?;
        self.n_measurements_entries.encode(dst)?;
        self.n_cs_entries.encode(dst)?;
        self.n_in_interests.encode(dst)?;
        self.n_in_data.encode(dst)?;
        self.n_in_nacks.encode(dst)?;
        self.n_out_interests.encode(dst)?;
        self.n_out_data.encode(dst)?;
        self.n_out_nacks.encode(dst)?;
        self.n_satisfied_interests.encode(dst)?;
        self.n_unsatisfied_interests.encode(dst)?;
        Ok(())
    }

    fn decode(src: &mut bytes::BytesMut) -> Result<Self, Self::Error> {
        Ok(Self {
            version: tlv::core::TlvCodec::decode(src)?,
            start_timestamp: tlv::core::TlvCodec::decode(src)?,
            current_timestamp: tlv::core::TlvCodec::decode(src)?,
            n_name_tree_entries: tlv::core::TlvCodec::decode(src)?,
            n_fib_entries: tlv::core::TlvCodec::decode(src)?,
            n_pit_entries: tlv::core::TlvCodec::decode(src)?,
            n_measurements_entries: tlv::core::TlvCodec::decode(src)?,
            n_cs_entries: tlv::core::TlvCodec::decode(src)?,
            n_in_interests: tlv::core::TlvCodec::decode(src)?,
            n_in_data: tlv::core::TlvCodec::decode(src)?,
            n_in_nacks: tlv::core::TlvCodec::decode(src)?,
            n_out_interests: tlv::core::TlvCodec::decode(src)?,
            n_out_data: tlv::core::TlvCodec::decode(src)?,
            n_out_nacks: tlv::core::TlvCodec::decode(src)?,
            n_satisfied_interests: tlv::core::TlvCodec::decode(src)?,
            n_unsatisfied_interests: tlv::core::TlvCodec::decode(src)?,
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
tlv::non_negative_number!(NSatisfiedInterests => tlv::Type::NSatisfiedInterests); // number of satisfied Interests, incremented when a PIT entry that has been satisfied is being removed
tlv::non_negative_number!(NUnsatisfiedInterests => tlv::Type::NUnsatisfiedInterests); // number of unsatisfied Interests, incremented when a PIT entry that has not been satisfied is being removed
