#include "mainwindow.h"
#include <QPushButton>
#include <QDebug>
#include <QDialog>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent) {
    setWindowTitle("My window title");
    resize(400, 300);
    auto button = new QPushButton("button_label", this);
    button->setGeometry(200-120/2, 150-60/2, 120, 60);
    connect(button, SIGNAL(clicked()), this, SLOT(clicked_handler()));
}

MainWindow::~MainWindow() {}

void MainWindow::clicked_handler() {
    // QDebug auto add newline
    qWarning() << "button clicked!";
    auto dialog = new QDialog(this);
    dialog->resize(400, 300);
    dialog->setWindowTitle("dialog");
    int dialog_res = dialog->exec();
    switch (dialog_res) {
    case QDialog::DialogCode::Rejected:
        qWarning() << "You close the dialog!";
        qWarning() << "QDialog::DialogCode::Rejected";
        break;
    case QDialog::DialogCode::Accepted:
        qWarning() << "QDialog::DialogCode::Accepted";
        break;
    default:
        qWarning() << "dialog_res =" << dialog_res;
        break;
    }
}
