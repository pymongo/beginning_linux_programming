#include "mainwindow.h"
#include <QPushButton>
#include <QDebug>
#include <iostream>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
{
    qDebug() << "MainWindow" << this << "at position" << this->pos();
    this->setWindowTitle("My window title");
    auto button = new QPushButton(this);
    connect(button, SIGNAL(clicked()), this, SLOT(my_clicked()));
}

MainWindow::~MainWindow()
{
}

void MainWindow::my_clicked() {
    // QDebug auto add newline
    qWarning() << "button clicked!";
}
