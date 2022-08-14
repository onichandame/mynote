import 'package:flutter/material.dart';
import 'package:notebook/providers/global.dart';
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
        initialRoute: '/home',
        routes: getRoutes(),
      ),
    );
  }
}
