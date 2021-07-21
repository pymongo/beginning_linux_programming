#include <iostream>
#include <thread>
#include <chrono>
#include <ctime>

std::string now_ymd_hms() {
    char buf[] = "1970-01-01 00:00:00";
    const time_t now = time(nullptr);
    strftime(buf, sizeof(buf), "%Y-%m-%d %H:%M:%S", localtime(&now));
    return buf;
}

[[noreturn]] void run_interval(unsigned int interval_ms) {
    while (true) {
        std::cout << now_ymd_hms() << std::endl;
        std::this_thread::sleep_for(std::chrono::milliseconds(interval_ms));
    }
}

int main() {
    std::thread thread_2(run_interval, 1000);
    thread_2.join();
    return 0;
}
