#include <gtk/gtk.h>

void window_destroy(const GtkWidget *window, gpointer data) {
    gtk_main_quit();
}

int main(int argc, char *argv[]) {
    gtk_init(&argc, &argv);
    GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
    g_signal_connect (window, "destroy",
                      G_CALLBACK(window_destroy), NULL);

    // page_increment 和 page_size 对 spin_button 来说没用的参数，设置成 step_increment 一样就行了
    GtkAdjustment *adjustment = gtk_adjustment_new(100.0, 50.0, 150.0, 0.5, 0.5, 0.5);
    // spin_button is similar to HTML <input type="number">
    // climb_rate: 当按住数字增加按键不放时，数字增加的加速度，一般设置成 0 即可，不需要数值变化的加速度
    // digits: 小数点后面几位
    GtkWidget* spin_button = gtk_spin_button_new(GTK_ADJUSTMENT(adjustment), 0, 1);

    gtk_container_add(GTK_CONTAINER(window), spin_button);
    gtk_widget_show_all(window);
    gtk_main();

    return 0;
}
