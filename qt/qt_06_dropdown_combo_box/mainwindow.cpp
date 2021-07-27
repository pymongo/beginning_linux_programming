#include "mainwindow.h"
#include <QWidget>
#include <QLayout>
#include <QComboBox>
#include <QDebug>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
{
    auto root_widget = new QWidget(this);
    setCentralWidget(root_widget);
    auto vbox = new QVBoxLayout(root_widget);

    auto editable_combo = new QComboBox(root_widget);
    // user can input to search item in dropdown
    editable_combo->setEditable(true);
    vbox->addWidget(editable_combo);
    auto read_only_combo = new QComboBox(root_widget);
    read_only_combo->setEditable(false);
    vbox->addWidget(read_only_combo);

    const auto items = QStringList() << "China" << "UK";
    editable_combo->addItems(items);
    read_only_combo->addItems(items);
    read_only_combo->addItem("USA");
    read_only_combo->setCurrentIndex(2);

    connect(editable_combo, SIGNAL(currentIndexChanged(int)), this, SLOT(current_index_changed_callback(int)));
    connect(read_only_combo, SIGNAL(currentIndexChanged(int)), this, SLOT(current_index_changed_callback(int)));
}

MainWindow::~MainWindow() {}

void MainWindow::current_index_changed_callback(int index) {
    // get the text using itemText(index)
    qDebug() << "you select index" << index;
}
