#include <chrono>
#include <functional>
#include <memory>
#include <string>

#include "rclcpp/rclcpp.hpp"
#include "std_msgs/msg/string.hpp"
#include "jcan/jcan.h"

using namespace std::chrono_literals;
using namespace org::jcan;

/* This example creates a subclass of Node and uses std::bind() to register a
* member function as a callback from the timer. */

class MinimalPublisher : public rclcpp::Node
{
  public:
    MinimalPublisher()
    : Node("can_publisher"), count_(0)
    {
      canbus_ = open_bus("vcan0").into_raw();
      publisher_ = this->create_publisher<std_msgs::msg::String>("topic", 10);
      timer_ = this->create_wall_timer(
      500ms, std::bind(&MinimalPublisher::timer_callback, this));
    }

  private:
    void timer_callback()
    {
      auto message = std_msgs::msg::String();
      message.data = "Hello, world! " + std::to_string(count_++);
      RCLCPP_INFO(this->get_logger(), "Publishing: '%s'", message.data.c_str());
      publisher_->publish(message);

      // Make frame
      Frame frame;
      frame.id = 0x42;
      frame.data.push_back(0x01);
      frame.data.push_back(0x02);
      frame.data.push_back(0x03);
      frame.data.push_back(0x04);
      // Print
      RCLCPP_INFO(this->get_logger(), "Sending Frame: '%s'", frame.to_string().c_str());
      // Send!!
      canbus_->send(frame); 
    }
    rclcpp::TimerBase::SharedPtr timer_;
    rclcpp::Publisher<std_msgs::msg::String>::SharedPtr publisher_;
    size_t count_;
    Bus *canbus_;
};

int main(int argc, char * argv[])
{
  rclcpp::init(argc, argv);
  rclcpp::spin(std::make_shared<MinimalPublisher>());
  rclcpp::shutdown();
  return 0;
}
