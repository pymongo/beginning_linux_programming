#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <KXmlGuiWindow> // need to add `QT += KXmlGui` to qmake

class MainWindow : public KXmlGuiWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();
};
#endif // MAINWINDOW_H
