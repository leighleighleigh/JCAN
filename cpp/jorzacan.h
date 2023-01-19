#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


struct JorzaBus;

struct JorzaFrame;


extern "C" {

void jorzacan_bus_destroy(JorzaBus *bus);

JorzaBus *jorzacan_bus_new(const char *iface);

JorzaFrame *jorzacan_bus_receive(JorzaBus *bus);

void jorzacan_bus_send(JorzaBus *bus, const JorzaFrame *frame);

void jorzacan_bus_send_raw(JorzaBus *bus, uint32_t id, const uint8_t *data, size_t len);

void jorzacan_frame_data(const JorzaFrame *frame, uint8_t *data);

void jorzacan_frame_destroy(JorzaFrame *frame);

uint8_t jorzacan_frame_dlc(const JorzaFrame *frame);

uint32_t jorzacan_frame_id(const JorzaFrame *frame);

JorzaFrame *jorzacan_frame_new(uint32_t id, const uint8_t *data, size_t len);

char *jorzacan_frame_str(const JorzaFrame *frame);

} // extern "C"
