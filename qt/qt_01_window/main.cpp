#include <QApplication>
#include <QMainWindow>

// g++ main.cpp `pkgconf --cflags --libs Qt5Widgets` -fPIC
int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    auto window = new QMainWindow();
    window->show();
    return app.exec();
}
