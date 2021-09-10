use super::*;

#[test]
fn all_registers() {
    assert_eq!(RegAddr(0x19), SampleRateDivider::ADDR);
    assert_eq!(RegAddr(0x1A), Configure::ADDR);
    assert_eq!(RegAddr(0x1B), GyroConfig::ADDR);
    assert_eq!(RegAddr(0x1C), AccelConfig::ADDR);
    assert_eq!(RegAddr(0x23), FifoEnable::ADDR);
    assert_eq!(RegAddr(0x38), IntEnable::ADDR);
    assert_eq!(RegAddr(0x3B), AccelData::ADDR);
    assert_eq!(RegAddr(0x41), Temperature::ADDR);
    assert_eq!(RegAddr(0x43), GyroData::ADDR);
    assert_eq!(RegAddr(0x6A), UserCtrl::ADDR);
    assert_eq!(RegAddr(0x6B), PwrMgmt1::ADDR);
    assert_eq!(RegAddr(0x72), FifoCount::ADDR);
    assert_eq!(RegAddr(0x74), FifoData::ADDR);
}
