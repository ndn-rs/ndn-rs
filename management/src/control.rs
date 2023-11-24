use super::*;

#[derive(Debug)]
pub struct ControlParameters {
    pub name: Option<tlv::Name>,
    pub face_id: Option<face::FaceId>,
    pub uri: Option<face::Uri>,
    pub local_uri: Option<face::LocalUri>,
    pub origin: Option<Origin>,
    pub cost: Option<Cost>,
    pub capacity: Option<Capacity>,
    pub count: Option<Count>,
    pub base_congestion_marking_interval: Option<face::BaseCongestionMarkingInterval>,
    pub default_congestion_threshold: Option<face::DefaultCongestionThreshold>,
    pub mtu: Option<face::Mtu>,
    pub flags: Option<face::Flags>,
    pub mask: Option<face::Mask>,
    pub strategy: Option<Strategy>,
    pub expiration_period: Option<face::ExpirationPeriod>,
    pub face_persistency: Option<face::FacePersistency>,
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
            self.face_persistency.value(),
        ];
        tlv::collect_to_bytes(items)
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
