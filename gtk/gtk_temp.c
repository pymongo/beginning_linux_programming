#include <gtk/gtk.h>
#include <stdio.h>

GtkWidget *toggle_button;
GtkWidget *check_button;
GtkWidget *radio_button_1, *radio_button_2;

void closeApp(GtkWidget *window, gpointer data) {
    gtk_main_quit();
}

void add_widget_with_label(GtkContainer *box, gchar *caption, GtkWidget *widget) {
    GtkWidget *label = gtk_label_new(caption);
    GtkWidget *hbox = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 4);
    gtk_box_set_homogeneous(GTK_BOX(hbox), TRUE);
//    GtkWidget *hbox = gtk_hbox_new(TRUE, 4);

    gtk_container_add(GTK_CONTAINER (hbox), label);
    gtk_container_add(GTK_CONTAINER (hbox), widget);

    gtk_container_add(box, hbox);
}

void print_active(char *button_name, GtkToggleButton *button) {
    gboolean active = gtk_toggle_button_get_active(button);

    printf("%s is %s\n", button_name, active ? "active" : "not active");
}

void button_clicked(GtkWidget *button, gpointer data) {
    print_active("Checkbutton", GTK_TOGGLE_BUTTON(check_button));
    print_active("Togglebutton", GTK_TOGGLE_BUTTON(toggle_button));
    print_active("Radiobutton1", GTK_TOGGLE_BUTTON(radio_button_1));
    print_active("Radiobutton2", GTK_TOGGLE_BUTTON(radio_button_2));
    printf("\n");
}

gint main(gint argc, gchar *argv[]) {
    GtkWidget *window;
    GtkWidget *button;
    GtkWidget *vbox;

    gtk_init(&argc, &argv);
    window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_default_size(GTK_WINDOW(window), 200, 200);
    g_signal_connect (window, "destroy",
                      G_CALLBACK(closeApp), NULL);

    button = gtk_button_new_with_label("Ok");
    toggle_button = gtk_toggle_button_new_with_label("Toggle");
    check_button = gtk_check_button_new();
    radio_button_1 = gtk_radio_button_new(NULL);
    radio_button_2 = gtk_radio_button_new_from_widget(GTK_RADIO_BUTTON(radio_button_1));

    vbox = gtk_vbox_new(TRUE, 4);
    add_widget_with_label(GTK_CONTAINER(vbox), "ToggleButton:", toggle_button);
    add_widget_with_label(GTK_CONTAINER(vbox), "CheckButton:", check_button);
    add_widget_with_label(GTK_CONTAINER(vbox), "Radio 1:", radio_button_1);
    add_widget_with_label(GTK_CONTAINER(vbox), "Radio 2:", radio_button_2);
    add_widget_with_label(GTK_CONTAINER(vbox), "Button:", button);

    g_signal_connect(button, "clicked",
                     G_CALLBACK(button_clicked), NULL);

    gtk_container_add(GTK_CONTAINER(window), vbox);
    gtk_widget_show_all(window);
    gtk_main();

    return 0;
}
