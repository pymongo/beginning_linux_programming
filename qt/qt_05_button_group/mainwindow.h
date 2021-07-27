#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QAbstractButton>
#include <QCheckBox>
#include <QRadioButton>
#include <QButtonGroup>

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();
private:
    void print_button_state(QAbstractButton *button);
    QCheckBox *checkbox;
    QRadioButton *radio_button_1, *radio_button_2;

private slots:
    void clicked_callback();
};
#endif // MAINWINDOW_H
