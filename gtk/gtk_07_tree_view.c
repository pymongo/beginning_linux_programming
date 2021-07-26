#include <gtk/gtk.h>

void close_app(const GtkWidget *window, gpointer data) {
    gtk_main_quit();
}

enum {
    COLUMN_USER_ID,
    COLUMN_USERNAME,
    N_COLUMN
};

int main(int argc, char *argv[]) {
    gtk_init(&argc, &argv);
    GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_position(GTK_WINDOW(window), GTK_WIN_POS_CENTER);
    g_signal_connect(window, "destroy", G_CALLBACK(close_app), NULL);

    // init tree_store model
    GtkTreeStore *store = gtk_tree_store_new(N_COLUMN, G_TYPE_INT64, G_TYPE_STRING);
    GtkTreeIter tree_iter;

    // init headers(column_name+column_type) and render
    GtkWidget *tree_view = gtk_tree_view_new_with_model(GTK_TREE_MODEL(store));
    GtkCellRenderer *row_render = gtk_cell_renderer_text_new();
    gtk_tree_view_insert_column_with_attributes(GTK_TREE_VIEW(tree_view), COLUMN_USER_ID, "user_id", row_render, "text",
                                                COLUMN_USER_ID, NULL);
    gtk_tree_view_insert_column_with_attributes(GTK_TREE_VIEW(tree_view), COLUMN_USERNAME, "username", row_render,
                                                "text", COLUMN_USERNAME, NULL);

    // insert data
    gtk_tree_store_append(store, &tree_iter, NULL); // prepare append a new row
    gtk_tree_store_set(store, &tree_iter, COLUMN_USER_ID, 1, COLUMN_USERNAME, "mike", -1);

    gtk_container_add(GTK_CONTAINER(window), tree_view);
    gtk_widget_show_all(window);
    gtk_main();
    return 0;
}
