#ifndef CNGUI_HPP
#define CNGUI_HPP

#include "cngui.h"
#include <string>

namespace cngui {

class Context {
public:
    Context(int width, int height, const std::string& title) {
        cn_init_params params = { sizeof(cn_init_params), title.c_str(), width, height };
        m_ctx = cn_init(&params);
    }

    ~Context() {
        if (m_ctx) cn_shutdown(m_ctx);
    }

    Context(const Context&) = delete;
    Context& operator=(const Context&) = delete;

    void begin_frame() { cn_begin_frame(m_ctx); }
    void end_frame() { cn_end_frame(m_ctx); }
    bool should_close() const { return cn_should_close(m_ctx) != 0; }

    void set_input(float x, float y, bool down, bool clicked) {
        cn_input_state input = { x, y, down ? 1 : 0, clicked ? 1 : 0 };
        cn_set_input(m_ctx, &input);
    }

    bool button(float x, float y, float w, float h, const std::string& label) {
        cn_rect bounds = { x, y, w, h };
        return cn_button(m_ctx, bounds, label.c_str()) != 0;
    }

private:
    cn_context* m_ctx = nullptr;
};

} // namespace cngui
#endif // CNGUI_HPP
