#include "mainwindow.h"
#include <KAboutApplicationDialog>
#include <KActionMenu>
#include <QDebug>

// https://develop.kde.org/docs/getting-started/main_window/
MainWindow::MainWindow(QWidget *parent) : KXmlGuiWindow(parent) {
    // add KDE application `Settings` and `Help` menu
    setupGUI();
}

MainWindow::~MainWindow() {}
