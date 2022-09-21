import 'package:flutter/material.dart';
import 'package:notebook/screens/dashboard/note/item.dart';
import 'package:notebook/screens/dashboard/note/list.dart';
import 'package:notebook/screens/dashboard/note/routes.dart';

class NoteMain extends StatelessWidget {
  const NoteMain({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Navigator(
      initialRoute: routeList,
      onGenerateRoute: (settings) {
        return MaterialPageRoute(
            builder: (context) {
              switch (settings.name) {
                case routeList:
                  return const NoteList();
                case routeItem:
                  return NoteItem(
                      id: (settings.arguments! as RouteItemArguments).id);
                default:
                  throw UnimplementedError(
                      'route ${settings.name} not implemented');
              }
            },
            settings: settings);
      },
    );
  }
}
