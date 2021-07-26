#include <QApplication>
#include <QMainWindow>

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);
    auto window = new QMainWindow();
    window->show();
    return app.exec();
}
