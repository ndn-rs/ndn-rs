use super::*;

#[derive(Debug)]
pub struct ControlParameters {
    name: Option<tlv::Name>,
    face_id: Option<face::FaceId>,
    uri: Option<face::Uri>,
    local_uri: Option<face::LocalUri>,
    origin: Option<Origin>,
    cost: Option<Cost>,
    capacity: Option<Capacity>,
    count: Option<Count>,
    base_congestion_marking_interval: Option<BaseCongestionMarkingInterval>,
    default_congestion_threshold: Option<DefaultCongestionThreshold>,
    mtu: Option<Mtu>,
    flags: Option<Flags>,
    mask: Option<Mask>,
    strategy: Option<Strategy>,
    expiration_period: Option<ExpirationPeriod>,
    face_prsistency: Option<face::FacePersistency>,
}

impl ControlParameters {}

impl tlv::Tlv for ControlParameters {
    fn r#type(&self) -> tlv::Type {
        tlv::Type::ControlParameters
    }

    fn value(&self) -> Option<Bytes> {
        let items = [
            self.name.value(),
            self.face_id.value(),
            self.uri.value(),
            self.local_uri.value(),
            self.origin.value(),
            self.cost.value(),
            self.capacity.value(),
            self.count.value(),
            self.base_congestion_marking_interval.value(),
            self.default_congestion_threshold.value(),
            self.mtu.value(),
            self.flags.value(),
            self.mask.value(),
            self.strategy.value(),
            self.expiration_period.value(),
            self.face_prsistency.value(),
        ];
        tlv::collect_to_bytes(items)
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
