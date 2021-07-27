#include "mainwindow.h"
#include <QPushButton>
#include <QDebug>
#include <iostream>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
{
    setWindowTitle("My window title");
    auto button = new QPushButton("button_label", this);
    connect(button, SIGNAL(clicked()), this, SLOT(my_clicked()));
}

MainWindow::~MainWindow()
{
}

void MainWindow::my_clicked() {
    // QDebug auto add newline
    qWarning() << "button clicked!";
}
