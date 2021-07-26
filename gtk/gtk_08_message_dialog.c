#include <gtk/gtk.h>
#include <stdio.h>

GtkWidget *main_window;

void button_clicked_callback(const GtkWidget *button, gpointer data) {
    GtkWidget *dialog = gtk_message_dialog_new(GTK_WINDOW(main_window), GTK_DIALOG_DESTROY_WITH_PARENT,
                                               GTK_MESSAGE_ERROR, GTK_BUTTONS_YES_NO, "are you sure?");
    int dialog_result = gtk_dialog_run(GTK_DIALOG(dialog));
    // or use dialog `response` signal/event callback function
    switch (dialog_result) {
        // GTK_RESPONSE_DELETE_EVENT == "user close the dialog"
        case GTK_RESPONSE_DELETE_EVENT:
            printf("GTK_RESPONSE_DELETE_EVENT\n");
            break;
        case GTK_RESPONSE_YES:
            printf("GTK_RESPONSE_YES\n");
            break;
        case GTK_RESPONSE_NO:
            printf("GTK_RESPONSE_NO\n");
            break;
        case GTK_RESPONSE_CLOSE:
            printf("GTK_RESPONSE_CLOSE\n");
            break;
        default:
            break;
    }
    printf("dialog_result = %d\n", dialog_result);
    gtk_widget_destroy(dialog);
}

void close_app(const GtkWidget *window_widget, gpointer data) {
    gtk_main_quit();
}

int main(int argc, char *argv[]) {
    gtk_init(&argc, &argv);

    main_window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(main_window), GTK_WIN_POS_CENTER);
    g_signal_connect(main_window, "destroy", G_CALLBACK(close_app), NULL);

    GtkWidget *button = gtk_button_new_with_label("my_button");
    gtk_container_add(GTK_CONTAINER(main_window), button);
    g_signal_connect(button, "clicked", G_CALLBACK(button_clicked_callback), NULL);

    gtk_widget_show_all(main_window);
    gtk_main();
    return 0;
}
