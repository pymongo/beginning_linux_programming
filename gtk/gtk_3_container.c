#include <gtk/gtk.h>

void window_destroy(GtkWidget *window, gpointer data) {
    printf("event: window destroy\n");
    gtk_main_quit();
}

// Callback allows the application to cancel a close/destroy event. (Return TRUE to cancel.)
gboolean window_delete_event(GtkWidget *widget, GdkEvent *event, gpointer data) {
    printf("event: window delete_event\n");
    return FALSE;
}

int main(int argc, char *argv[]) {
    gtk_init(&argc, &argv);
    GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);

    gtk_window_set_title(GTK_WINDOW(window), "The Window Title");
    gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(window), 300, 200);

    g_signal_connect (window, "destroy",
                      G_CALLBACK(window_destroy), NULL);
    g_signal_connect (window, "delete_event",
                      G_CALLBACK(window_delete_event), NULL);

    GtkWidget *vbox = gtk_box_new(GTK_ORIENTATION_VERTICAL, 10);
    GtkWidget *hbox = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 5);

    gtk_box_set_homogeneous(GTK_BOX(hbox), TRUE);
    gtk_box_pack_start(GTK_BOX(vbox), gtk_label_new("Label 1"), TRUE, FALSE, 5);
    gtk_box_pack_start(GTK_BOX(vbox), gtk_label_new("Label 2"), TRUE, FALSE, 5);
    gtk_box_pack_start(GTK_BOX(hbox), vbox, FALSE, FALSE, 5);
    gtk_box_pack_start(GTK_BOX(hbox), gtk_label_new("Label 3"), FALSE, FALSE, 5);
    gtk_container_add(GTK_CONTAINER(window), hbox);

    gtk_widget_show_all(window);
    gtk_main();

    return 0;
}
