#include <qapplication.h>
#include <qmainwindow.h>

int main(int argc, char **argv) {
    QApplication app(argc, argv);
    auto *window = new QMainWindow();
    window->show();
    // return app.exec();
    return QApplication::exec();
}
