import 'package:flutter/material.dart';
import 'package:notebook/providers/global.dart';
import 'package:notebook/screens/dashboard/dashboard.dart';
import 'package:notebook/screens/loading.dart';
import 'package:notebook/screens/login.dart';
import 'package:notebook/screens/settings/general/general.dart';
import 'package:notebook/screens/settings/profile/profile.dart';
import 'package:notebook/screens/routes.dart';
import 'package:notebook/screens/settings/settings.dart';
import 'package:notebook/screens/signup.dart';

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
        routes: {
          routeLoading: (context) => const LoadingScreen(),
          routeDashboard: (context) => const DashboardScreen(),
          routeSignup: (context) => const SignupScreen(),
          routeLogin: (context) => const LoginScreen(),
          routeSettings: (context) => const SettingsScreen(),
          routeSettingsGeneral: (context) => const GeneralSettingsScreen(),
          routeSettingsProfile: (context) => const ProfileScreen(),
        },
        initialRoute: routeLoading,
      ),
    );
  }
}
