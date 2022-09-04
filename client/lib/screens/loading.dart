import 'package:flutter/material.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class LoadingScreen extends StatelessWidget {
  const LoadingScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final client = Provider.of<Client?>(context);
    if (client != null) {
      Future.microtask(() => Navigator.of(context)
          .pushNamedAndRemoveUntil(routeHome, (_) => false));
    }
    return Scaffold(
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: const [Text("loading...")],
          ),
        ),
        backgroundColor: Colors.indigo);
  }
}
