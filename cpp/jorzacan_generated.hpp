#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace ffi {

struct JorzaBus;

struct JorzaFrame;

extern "C" {

JorzaBus *jorzacan_bus_new(const char *iface);

void jorzacan_bus_destroy(JorzaBus *bus);

JorzaFrame *jorzacan_frame_new(uint32_t id, const uint8_t *data, uintptr_t len);

void jorzacan_frame_destroy(JorzaFrame *frame);

void jorzacan_bus_send_raw(JorzaBus *bus, uint32_t id, const uint8_t *data, uintptr_t len);

void jorzacan_bus_send(JorzaBus *bus, const JorzaFrame *frame);

JorzaFrame *jorzacan_bus_receive(JorzaBus *bus);

uint32_t jorzacan_frame_id(const JorzaFrame *frame);

void jorzacan_frame_data(const JorzaFrame *frame, uint8_t *data);

uint8_t jorzacan_frame_dlc(const JorzaFrame *frame);

char *jorzacan_frame_str(const JorzaFrame *frame);

} // extern "C"

} // namespace ffi
