#include <gtk/gtk.h>
#include <stdio.h>

void button_clicked_callback(const GtkWidget *button, gpointer data) {
    printf("event=clicked, button receive data = %s\n", (char *) data);
}

void close_app(const GtkWidget *window, gpointer data) {
    gtk_main_quit();
}

int main(int argc, char *argv[]) {
    gtk_init(&argc, &argv);

    GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
    g_signal_connect(window, "destroy", G_CALLBACK(close_app), NULL);

    GtkWidget *button = gtk_button_new_with_label("my_button");
    gtk_container_add(GTK_CONTAINER(window), button);
    g_signal_connect(button, "clicked", G_CALLBACK(button_clicked_callback), "user input data send to button");

    gtk_widget_show(window);
    gtk_widget_show(button);
    gtk_main();
    return 0;
}
