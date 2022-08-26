import 'package:flutter/material.dart';
import 'package:notebook/providers/global.dart';
import 'package:notebook/screens/home.dart';
import 'package:notebook/screens/loading.dart';
import 'package:notebook/screens/routes.dart';

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return Global(
      child: MaterialApp(
        title: 'Notebook',
        theme: ThemeData(
          primarySwatch: Colors.indigo,
        ),
        debugShowCheckedModeBanner: false,
        onGenerateRoute: (settings) {
          late Widget page;
          if (settings.name == routeLoading) {
            page = const LoadingScreen();
          } else if (settings.name!.startsWith(routeHome)) {
            page = const HomeScreen();
          } else {
            throw Exception('404: Unknown route: ${settings.name}');
          }
          return MaterialPageRoute(
              builder: (context) => page, settings: settings);
        },
        initialRoute: routeLoading,
      ),
    );
  }
}
