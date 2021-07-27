#include "mainwindow.h"
#include <QPushButton>
#include <QDebug>
#include <QMessageBox>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent) {
    resize(400, 300);
    auto button = new QPushButton("button_label", this);
    button->setGeometry(200-120/2, 150-60/2, 120, 60);
    connect(button, SIGNAL(clicked()), this, SLOT(clicked_handler()));
}

MainWindow::~MainWindow() {}

void MainWindow::clicked_handler() {
    // QInputDialog::getText == QDialog + QLineEdit
    // QInputDialog::getItem == QDialog + QComboBox
    int res = QMessageBox::warning(this, "Title", "message", QMessageBox::Yes | QMessageBox::Default, QMessageBox::No  | QMessageBox::Escape);
    switch (res) {
    case QMessageBox::No:
        qWarning() << "You close the dialog!";
        qWarning() << "QMessageBox::No";
        break;
    case QMessageBox::Yes:
        qWarning() << "QMessageBox::Yes";
        break;
    default:
        qWarning() << "res =" << res;
        break;
    }
}
