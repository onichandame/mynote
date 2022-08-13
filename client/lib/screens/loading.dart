import 'package:flutter/material.dart';
import 'package:notebook/providers/client.dart';
import 'package:provider/provider.dart';

class LoadingScreen extends StatelessWidget {
  const LoadingScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: Center(
      child: Consumer<Client>(
        builder: (_, client, child) => const Text('loading'),
      ),
    ));
  }
}
