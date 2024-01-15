use super::*;

// BaseCongestionMarkingInterval indicates the base marking interval for congestion marking.
// The value of this attribute is the base marking interval in nanoseconds,
// which is used to compute the interval at which congested packets will be marked.
tlv::non_negative_number!(BaseCongestionMarkingInterval => tlv::Type::BaseCongestionMarkingInterval);

// DefaultCongestionThreshold indicates the default congestion threshold
// if the face does not support retrieving the send queue capacity.
// The value of this attribute is the default congestion threshold in bytes.
tlv::non_negative_number!(DefaultCongestionThreshold => tlv::Type::DefaultCongestionThreshold);
