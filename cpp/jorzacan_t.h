#ifndef JORZACAN_H
#define JORZACAN_H

#include <jorzacan.h>
#include <memory>

// using the default_delete template, used for construction of JorzaBus and JorzaFrame
template <>
struct std::default_delete<JorzaBus> {
    void operator()(JorzaBus *bus) {
        jorzacan_bus_destroy(bus);
    }
};

template <>
struct std::default_delete<JorzaFrame> {
    void operator()(JorzaFrame *frame) {
        jorzacan_frame_destroy(frame);
    }
};

// Define the JorzaFrame class, keeping a private unique pointer to the underlying JorzaFrame
class JorzaFrame {
private:
    std::unique_ptr<JorzaFrame, std::default_delete<JorzaFrame>> frame;
public:
    JorzaFrame(uint32_t id, const uint8_t *data, size_t len);
    uint32_t get_id();
    uint8_t *get_data();
    size_t get_len();
};

// Define the JorzaBus class, keeping a private unique pointer to the underlying JorzaBus
class JorzaBus {
private:
    std::unique_ptr<JorzaBus, std::default_delete<JorzaBus>> bus;
public:
    JorzaBus(const char *iface);
    void send(JorzaFrame *frame);
    void send_raw(uint32_t id, const uint8_t *data, size_t len);
    JorzaFrame *receive();
};

#endif // JORZACAN_H