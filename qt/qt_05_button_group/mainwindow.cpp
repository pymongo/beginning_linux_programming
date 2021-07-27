#include "mainwindow.h"
#include <QWidget>
#include <QLayout>
#include <QPushButton>
#include <QDebug>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
{
    auto root_widget = new QWidget(this);
    setCentralWidget(root_widget);
    auto vbox = new QVBoxLayout(root_widget);

    // checkbox has a optional third state "unchanged"
    this->checkbox = new QCheckBox(root_widget);
    this->checkbox->setObjectName("checkbox");
    vbox->addWidget(this->checkbox);

    auto button_group = new QButtonGroup(NULL);
    this->radio_button_1 = new QRadioButton(root_widget);
    this->radio_button_1->setObjectName("radio_button_1");
    button_group->addButton(this->radio_button_1);
    vbox->addWidget(this->radio_button_1);
    this->radio_button_2 = new QRadioButton(root_widget);
    this->radio_button_2->setObjectName("radio_button_2");
    button_group->addButton(this->radio_button_2);
    vbox->addWidget(this->radio_button_2);

    auto button = new QPushButton(root_widget);
    button->setText("print state");
    vbox->addWidget(button);

    connect(button, SIGNAL(clicked()), this, SLOT(clicked_callback()));
}

MainWindow::~MainWindow()
{
}

void MainWindow::clicked_callback() {
    print_button_state(checkbox);
    print_button_state(radio_button_1);
    print_button_state(radio_button_2);
}

void MainWindow::print_button_state(QAbstractButton *button) {
    if (button->isChecked()) {
        // qDebug auto add whitespace between `<<`, and auto add \n at end;
        qDebug() << button->objectName() << "is on";
    } else {
        qDebug() << button->objectName() << "is off";
    }
}
