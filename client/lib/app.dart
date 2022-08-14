import 'package:flutter/material.dart';
import 'package:notebook/providers/api_schema.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/screens/home.dart';
import 'package:notebook/screens/loading.dart';
import 'package:provider/provider.dart';
import 'package:shared_preferences/shared_preferences.dart';

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MultiProvider(
      providers: [
        FutureProvider(create: (_) => ApiSchema.create(), initialData: null),
        FutureProvider(
          create: (context) => SharedPreferences.getInstance(),
          initialData: null,
        ),
        ChangeNotifierProvider(
          create: (context) => Client(
              Provider.of<SharedPreferences?>(context, listen: false),
              Provider.of<ApiSchema?>(context, listen: false)),
        ),
        FutureProvider(
            create: (context) async {
              final client = Provider.of<Client>(context, listen: false);
              if (client.session == null) {
                return null;
              } else {
                return await client.getUser();
              }
            },
            initialData: null)
      ],
      child: MaterialApp(
        title: 'Notebook',
        theme: ThemeData(
          primarySwatch: Colors.indigo,
        ),
        initialRoute: '/home',
        routes: {
          '/home': (_) => const HomeScreen(),
          '/loading': (_) => const LoadingScreen(),
        },
      ),
    );
  }
}
