import 'dart:developer';

import 'package:flutter/material.dart';
import 'package:notebook/providers/client.dart';
import 'package:provider/provider.dart';

class SignupScreen extends StatefulWidget {
  const SignupScreen({Key? key}) : super(key: key);

  @override
  State<SignupScreen> createState() => _SignupScreenState();
}

class _SignupScreenState extends State<SignupScreen> {
  final _formKey = GlobalKey<FormState>();
  bool _busy = false;
  String? _name;
  String? _password;
  String? _email;
  @override
  Widget build(BuildContext context) {
    final client = Provider.of<Client>(context);
    return Scaffold(
        body: Form(
      key: _formKey,
      child: Center(
        child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 256),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                TextFormField(
                  decoration: const InputDecoration(
                    labelText: "Name",
                  ),
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return 'Enter your name';
                    }
                    return null;
                  },
                  onSaved: (value) {
                    _name = value;
                  },
                ),
                TextFormField(
                  decoration: const InputDecoration(
                    labelText: "Password",
                  ),
                  validator: (value) {
                    if (value == null || value.isEmpty) return 'Enter password';
                    if (value.length < 5) return 'minimum 5 characters';
                    if (value.length > 10) return 'maximum 10 characters';
                    return null;
                  },
                  onSaved: (value) {
                    _password = value;
                  },
                ),
                Padding(
                    padding: const EdgeInsets.symmetric(vertical: 16),
                    child: ElevatedButton(
                        onPressed: _busy
                            ? null
                            : () {
                                if (_formKey.currentState!.validate()) {
                                  _formKey.currentState!.save();
                                  ScaffoldMessenger.of(context)
                                      .hideCurrentSnackBar();
                                  setState(() {
                                    _busy = true;
                                  });
                                  client
                                      .signup(
                                          name: _name!, password: _password!)
                                      .then((value) {
                                    client.session = value;
                                    ScaffoldMessenger.of(context).showSnackBar(
                                        const SnackBar(
                                            content: Text('signed up')));
                                  }).catchError((e) {
                                    ScaffoldMessenger.of(context).showSnackBar(
                                        SnackBar(content: Text(e.toString())));
                                  }).whenComplete(() {
                                    setState(() {
                                      _busy = false;
                                    });
                                  });
                                }
                              },
                        child: const Text("SIGN UP")))
              ],
            )),
      ),
    ));
  }
}
