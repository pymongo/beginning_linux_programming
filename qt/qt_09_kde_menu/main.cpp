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
        QStringLiteral("http://example.com/"),
        QStringLiteral("submit@bugs.kde.org")
    );
    aboutData.addAuthor("Name", "Author Role", QStringLiteral("your@email.com"),
                         QStringLiteral("http://your.website.com"), QStringLiteral("OCS Username"));
    KAboutData::setApplicationData(aboutData);

    MainWindow *window = new MainWindow();
    window->show();
    return app.exec();
}
