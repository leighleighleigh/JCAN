#include "jorzacan_t.h"

// Implement the class instantiation calls
JorzaBus::JorzaBus(const char *iface) {
    bus = std::unique_ptr<JorzaBus, std::default_delete<JorzaBus>>(jorzacan_bus_new(iface));
}

JorzaFrame::JorzaFrame(uint32_t id, const uint8_t *data, size_t len) {
    frame = std::unique_ptr<JorzaFrame, std::default_delete<JorzaFrame>>(jorzacan_frame_new(id, data, len));
}


// Implement the class methods for JorzaBus
void JorzaBus::send(JorzaFrame *frame) {
    jorzacan_bus_send(bus.get(), frame);
}

// sendRaw
void JorzaBus::send_raw(uint32_t id, const uint8_t *data, size_t len) {
    jorzacan_bus_send_raw(bus.get(), id, data, len);
}

// receive
JorzaFrame * JorzaBus::receive() {
    // Need to pass self to the bus receive method, and return a JorzaFrame
    return jorzacan_bus_receive(bus.get());
}

// Implement the class methods for JorzaFrame
// get_id
uint32_t JorzaFrame::get_id() {
    return jorzacan_frame_id(frame.get());
}

// get_data, allocating an array of DLC bytes
uint8_t *JorzaFrame::get_data() {
    uint8_t *data = new uint8_t[jorzacan_frame_dlc(frame.get())];
    jorzacan_frame_data(frame.get(), data);
    return data;
}

// get_len
size_t JorzaFrame::get_len() {
    return jorzacan_frame_dlc(frame.get());
}
