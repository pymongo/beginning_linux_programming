#include <gtk/gtk.h>
#include <stdio.h>
#include <string.h>

void window_close(const GtkWidget *window, gpointer data) {
    gtk_main_quit();
}

void button_clicked(const GtkWidget *button, gpointer data) {
    const char *password_text = gtk_entry_get_text(GTK_ENTRY((GtkWidget *) data));
    if (strcmp(password_text, "secret") == 0) {
        printf("Access granted!\n");
    } else {
        printf("Access denied!\n");
    }
}

int main(int argc, char *argv[]) {
    gtk_init(&argc, &argv);

    GtkWidget* window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_title(GTK_WINDOW(window), "GtkEntryBox");
    gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
    gtk_window_set_default_size(GTK_WINDOW(window), 200, 200);
    g_signal_connect (window, "destroy",
                      G_CALLBACK(window_close), NULL);

    GtkWidget *username_row = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 5);
    GtkWidget *password_row = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 5);
    GtkWidget* password_entry = gtk_entry_new();
    gtk_entry_set_visibility(GTK_ENTRY (password_entry), FALSE);
    gtk_box_set_homogeneous(GTK_BOX(username_row), TRUE);
    gtk_box_set_homogeneous(GTK_BOX(password_row), TRUE);
    GtkWidget* ok_button = gtk_button_new_with_label("Ok");
    g_signal_connect (ok_button, "clicked",
                      G_CALLBACK(button_clicked), password_entry);

    GtkWidget *vbox = gtk_box_new(GTK_ORIENTATION_VERTICAL, 10);

    gtk_box_pack_start(GTK_BOX(username_row), gtk_label_new("Login:"), TRUE, FALSE, 5);
    gtk_box_pack_start(GTK_BOX(username_row), gtk_entry_new(), TRUE, FALSE, 5);

    gtk_box_pack_start(GTK_BOX(password_row), gtk_label_new("Password:"), TRUE, FALSE, 5);
    gtk_box_pack_start(GTK_BOX(password_row), password_entry, TRUE, FALSE, 5);

    gtk_box_pack_start(GTK_BOX(vbox), username_row, FALSE, FALSE, 5);
    gtk_box_pack_start(GTK_BOX(vbox), password_row, FALSE, FALSE, 5);
    gtk_box_pack_start(GTK_BOX(vbox), ok_button, FALSE, FALSE, 5);

    gtk_container_add(GTK_CONTAINER(window), vbox);

    gtk_widget_show_all(window);
    gtk_main();

    return 0;
}
