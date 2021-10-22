#include "api/rtp_packet_info.h"

namespace webrtc {
class RTC_EXPORT RtpPacketInfos {
 public:
  using const_iterator = std::vector<RtpPacketInfo>::const_iterator;
};
}