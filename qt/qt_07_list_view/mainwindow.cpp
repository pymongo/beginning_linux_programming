#include "mainwindow.h"
#include <QVBoxLayout>
#include <QListView>
#include <QStringListModel>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent) {
    // list_view == tree_view
    auto list_view = new QListView(this);
    auto list_model = new QStringListModel(this);
    list_model->setStringList(QStringList() << "China" << "UK");
    list_view->setModel(list_model);
//    this->resize(300, 400);
    list_view->resize(300, 400);
}

MainWindow::~MainWindow(){}
