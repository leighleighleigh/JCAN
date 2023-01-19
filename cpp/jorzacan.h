#include <jorzacan_generated.hpp>
#include <memory>

// using the default_delete template, used for construction of JorzaBus and JorzaFrame
template <>
struct std::default_delete<ffi::JorzaBus> {
    void operator()(ffi::JorzaBus *bus) {
        ffi::jorzacan_bus_destroy(bus);
    }
};

template <>
struct std::default_delete<ffi::JorzaFrame> {
    void operator()(ffi::JorzaFrame *frame) {
        ffi::jorzacan_frame_destroy(frame);
    }
};

// Define the JorzaFrame class, keeping a private unique pointer to the underlying JorzaFrame
class JorzaFrame {
public:
    JorzaFrame(uint32_t id, const uint8_t *data, size_t len);
    uint32_t get_id();
    uint8_t *get_data();
    size_t get_len();
    std::unique_ptr<ffi::JorzaFrame, std::default_delete<ffi::JorzaFrame>> frame;
};

// Define the JorzaBus class, keeping a private unique pointer to the underlying JorzaBus
class JorzaBus {
private:
    std::unique_ptr<ffi::JorzaBus, std::default_delete<ffi::JorzaBus>> bus;
public:
    JorzaBus(const char *iface);
    void send(JorzaFrame *frame);
    void send_raw(uint32_t id, const uint8_t *data, size_t len);
    JorzaFrame *receive();
};


// Implement the class instantiation calls
JorzaBus::JorzaBus(const char *iface) {
    bus = std::unique_ptr<ffi::JorzaBus, std::default_delete<ffi::JorzaBus>>(ffi::jorzacan_bus_new(iface));
}

JorzaFrame::JorzaFrame(uint32_t id, const uint8_t *data, size_t len) {
    frame = std::unique_ptr<ffi::JorzaFrame, std::default_delete<ffi::JorzaFrame>>(ffi::jorzacan_frame_new(id, data, len));
}

// Implement the class methods for JorzaBus
void JorzaBus::send(JorzaFrame *frame) {
    ffi::jorzacan_bus_send(bus.get(), frame->frame.get());
}

// sendRaw
void JorzaBus::send_raw(uint32_t id, const uint8_t *data, size_t len) {
    ffi::jorzacan_bus_send_raw(bus.get(), id, data, len);
}

// receive
JorzaFrame * JorzaBus::receive() {
    // Receve an ffi::JorzaFrame struct, then intiialise a new JorzaFrame object using the id,data,len method
    ffi::JorzaFrame *frame = ffi::jorzacan_bus_receive(bus.get());
    // Extract the data using ffi::jorzacan_frame_data. This requires a buffer of DLC bytes
    uint8_t *data = new uint8_t[ffi::jorzacan_frame_dlc(frame)];
    ffi::jorzacan_frame_data(frame, data);
    // Return a new JorzaFrame object
    return new JorzaFrame(ffi::jorzacan_frame_id(frame), data, ffi::jorzacan_frame_dlc(frame));
}

// Implement the class methods for JorzaFrame
// get_id
uint32_t JorzaFrame::get_id() {
    return ffi::jorzacan_frame_id(frame.get());
}

// get_data, allocating an array of DLC bytes
uint8_t *JorzaFrame::get_data() {
    uint8_t *data = new uint8_t[ffi::jorzacan_frame_dlc(frame.get())];
    ffi::jorzacan_frame_data(frame.get(), data);
    return data;
}

// get_len
size_t JorzaFrame::get_len() {
    return ffi::jorzacan_frame_dlc(frame.get());
}