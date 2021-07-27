#include "mainwindow.h"
#include <QLayout>
#include <QLabel>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
{
    QWidget *widget = new QWidget();
    this->setCentralWidget(widget);
    QHBoxLayout *root = new QHBoxLayout(widget);
    root->setSpacing(0);

    auto col_1 = new QVBoxLayout();
    col_1->setSpacing(0);
//    col_1->setMargin(0);
    auto label_1 = new QLabel("label_1");
    label_1->setStyleSheet("color: red; background-color: yellow");
    label_1->setAlignment(Qt::AlignCenter);
    auto label_2 = new QLabel("label_2");
    label_2->setStyleSheet("color: aqua; background-color: gray");
    label_2->setAlignment(Qt::AlignCenter);
    col_1->addWidget(label_1);
    col_1->addWidget(label_2);

    auto label_3 = new QLabel("label_3");
    label_3->setStyleSheet("color: aqua; background-color: pink");
    label_3->setAlignment(Qt::AlignCenter);
    root->addLayout(col_1);
    root->addWidget(label_3);

    this->resize(400, 300);
}

MainWindow::~MainWindow()
{
}

