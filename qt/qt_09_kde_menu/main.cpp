#include "mainwindow.h"
#include <QApplication>
#include <KAboutData>
#include <KMessageBox>

int main (int argc, char *argv[]) {
    QApplication app(argc, argv);

    KAboutData aboutData(
        QStringLiteral("The program name used internally. (componentName)"),
        "qt_09_kde_menu",
        QStringLiteral("0.1.0"),
        "shortDescription of qt_09_kde_menu",
        KAboutLicense::GPL_V2,
        "copyright (c) 2021",
        "Optional text shown in the About box",
        QStringLiteral("https://github.com/pymongo/beginning_linux_programming"),
        QStringLiteral("submit@bugs.kde.org")
    );
    aboutData.addAuthor("Name", "Author Role", QStringLiteral("your@email.com"),
                         QStringLiteral("https://github.com/pymongo/beginning_linux_programming"), QStringLiteral("OCS Username"));
    KAboutData::setApplicationData(aboutData);

    auto window = new MainWindow();
    window->show();
    return app.exec();
}
