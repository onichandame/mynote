import 'package:flutter/material.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class LoadingScreen extends StatelessWidget {
  static const routeName = routeLoading;

  const LoadingScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final client = Provider.of<Client?>(context);
    if (client != null) {
      Future.microtask(
          () => Navigator.pushReplacementNamed(context, routeHome));
    }
    return const Scaffold(
        body: Center(
          child: Text("loading..."),
        ),
        backgroundColor: Colors.indigo);
  }
}
