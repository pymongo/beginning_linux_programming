#include "mainwindow.h"
#include <QLabel>
#include <QGridLayout>
#include <QLineEdit>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
{
    auto root_widget = new QWidget(this);
    setCentralWidget(root_widget);
    auto grid = new QGridLayout(root_widget);
    grid->addWidget(new QLabel("amount: ", root_widget), 0, 0);
    grid->addWidget(new QLabel("password: ", root_widget), 1, 0);

    auto amount_entry = new QLineEdit(root_widget);
    // amount_entry.setValidator(new QRegExpValidator(QRegExp("[0-9]*"), &amount_entry));
    amount_entry->setInputMask("999"); // require 1-3 digits
    grid->addWidget(amount_entry, 0, 1);

    auto password_entry = new QLineEdit(root_widget);
    password_entry->setEchoMode(QLineEdit::Password);
    grid->addWidget(password_entry, 1, 1);
}

MainWindow::~MainWindow()
{
}

