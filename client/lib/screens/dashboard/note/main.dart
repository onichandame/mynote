import 'package:flutter/material.dart';
import 'package:notebook/screens/dashboard/note/item.dart';
import 'package:notebook/screens/dashboard/note/list.dart';
import 'package:notebook/screens/dashboard/note/routes.dart';

class NoteMain extends StatelessWidget {
  const NoteMain({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final routes = _getRoutes(context);
    return Navigator(
      initialRoute: routeList,
      onGenerateRoute: (settings) {
        return MaterialPageRoute(
            builder: (context) => routes[settings.name]!(context),
            settings: settings);
      },
    );
  }

  Map<String, WidgetBuilder> _getRoutes(BuildContext context) {
    return {
      routeList: (context) => const NoteList(),
      routeItem: (context) => const NoteItem(),
    };
  }
}
