#include <gtk/gtk.h>
#include <stdio.h>

GtkWidget *toggle_button;
GtkWidget *check_button;
GtkWidget *radio_button_1, *radio_button_2; // all radio_button in a same group

void closeApp(GtkWidget *window, gpointer data) {
    gtk_main_quit();
}

void add_widget_with_label(GtkContainer *box, gchar *label_text, GtkWidget *widget) {
    GtkWidget *label = gtk_label_new(label_text);
    GtkWidget *hbox = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 4);
    gtk_box_set_homogeneous(GTK_BOX(hbox), TRUE);

    gtk_container_add(GTK_CONTAINER (hbox), label);
    gtk_container_add(GTK_CONTAINER (hbox), widget);
    gtk_container_add(box, hbox);
}

void print_active(const char *button_name, GtkToggleButton *button) {
    gboolean active = gtk_toggle_button_get_active(button);
    printf("%s is %s\n", button_name, active ? "true" : "false");
}

void button_clicked(const GtkWidget *button, gpointer data) {
    print_active("Togglebutton", GTK_TOGGLE_BUTTON(toggle_button));
    print_active("Checkbutton ", GTK_TOGGLE_BUTTON(check_button));
    print_active("Radiobutton1", GTK_TOGGLE_BUTTON(radio_button_1));
    print_active("Radiobutton2", GTK_TOGGLE_BUTTON(radio_button_2));
    printf("\n");
}

gint main(gint argc, gchar *argv[]) {
    gtk_init(&argc, &argv);
    GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_default_size(GTK_WINDOW(window), 200, 200);
    g_signal_connect (window, "destroy",
                      G_CALLBACK(closeApp), NULL);
    GtkWidget *button = gtk_button_new_with_label("print_state");
    g_signal_connect(button, "clicked",
                     G_CALLBACK(button_clicked), NULL);
    toggle_button = gtk_toggle_button_new_with_label("toggle");
    check_button = gtk_check_button_new();
    radio_button_1 = gtk_radio_button_new(NULL);
    radio_button_2 = gtk_radio_button_new_from_widget(GTK_RADIO_BUTTON(radio_button_1));

    GtkWidget *root_vbox = gtk_box_new(GTK_ORIENTATION_VERTICAL, 4);
    add_widget_with_label(GTK_CONTAINER(root_vbox), "ToggleButton:", toggle_button);
    add_widget_with_label(GTK_CONTAINER(root_vbox), "CheckButton:", check_button);
    add_widget_with_label(GTK_CONTAINER(root_vbox), "Radio 1:", radio_button_1);
    add_widget_with_label(GTK_CONTAINER(root_vbox), "Radio 2:", radio_button_2);
    add_widget_with_label(GTK_CONTAINER(root_vbox), "print_state:", button);
    gtk_container_add(GTK_CONTAINER(window), root_vbox);
    gtk_widget_show_all(window);
    gtk_main();
    return 0;
}
