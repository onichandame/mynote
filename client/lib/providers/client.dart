import 'dart:convert';
import 'dart:html';

import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:graphql/client.dart';
import 'package:image_picker/image_picker.dart';
import 'package:notebook/models/user.dart';
import 'package:shared_preferences/shared_preferences.dart';

class Client extends ChangeNotifier with http.BaseClient {
  final SharedPreferences sharedPrefs;

  static final Future<String> _schemaState =
      rootBundle.loadString('assets/api.graphql');
  static const String _sessionKey = "auth";
  static const String _urlKey = "url";

  late String _url;
  String? _session;

  Client(
    this.sharedPrefs,
  ) {
    session = sharedPrefs.getString(_sessionKey);
    url = sharedPrefs.getString(_urlKey) ??
        const String.fromEnvironment('API_URL',
            defaultValue: 'http://localhost');
  }

  String get url => _url;

  set url(String value) {
    _url = value.replaceAll(
        RegExp(
          r'\/$',
        ),
        '');
    notifyListeners();
    sharedPrefs.setString(_urlKey, _url);
  }

  String get apiUrl => '$url/api';

  String get contentUrl => '$url/content';

  GraphQLClient get apiClient {
    final httpLink = HttpLink(apiUrl);
    final wsLink = WebSocketLink(apiUrl);
    final transportLink =
        Link.split((request) => request.isSubscription, wsLink, httpLink);
    final authLink =
        AuthLink(getToken: () => _session == null ? null : 'Bearer $_session');
    final link = authLink.concat(transportLink);
    return GraphQLClient(link: link, cache: GraphQLCache());
  }

  @override
  Future<http.StreamedResponse> send(http.BaseRequest request) {
    if (session != null) request.headers['authorization'] = 'Bearer $session';
    return http.Client().send(request);
  }

  String? get session => _session;

  set session(String? sess) {
    bool changed = sess != _session;
    _session = sess;
    if (changed) {
      if (sess == null) {
        sharedPrefs.remove(_sessionKey);
      } else {
        sharedPrefs.setString(_sessionKey, sess);
      }
      notifyListeners();
    }
  }

  Future<User?> getUser() async {
    if (session == null) return null;
    return User.fromJson(
        (await _request(operationName: 'users'))?['edges']?[0]?['node']);
  }

  Future<String> signup(
      {required String name, required String password, String? email}) async {
    return await _request(
        operationName: 'signup',
        variables: {'name': name, 'password': password, 'email': email});
  }

  Future<String> login(
      {required String identity, required String password}) async {
    return await _request(
        operationName: 'login',
        variables: {'identity': identity, 'password': password});
  }

  Future<String> renewSession() async {
    return await _request(operationName: 'renewSession');
  }

  Future<void> updateSelf({String? name, String? email, String? avatar}) async {
    await _request(
        operationName: 'updateSelf',
        variables: {'name': name, 'email': email, 'avatar': avatar});
  }

  Future<dynamic> _request(
      {required String operationName,
      Map<String, dynamic> variables = const {},
      String? resultName}) async {
    variables.removeWhere((key, value) => value == null);
    final response = await apiClient.query(QueryOptions(
        document: gql(await _schemaState),
        operationName: operationName,
        variables: variables));
    final error = response.exception;
    if (error != null) throw error;
    final key = resultName ?? operationName;
    final result = response.data?[key];
    if (result == null) throw Exception('$key not found');
    return result;
  }

  Future<String> uploadFile(XFile file) async {
    final request = http.MultipartRequest('POST', Uri.parse(contentUrl));
    final uploadedFile = http.MultipartFile.fromBytes(
        'file', await file.readAsBytes(),
        filename: file.name);
    request.files.add(uploadedFile);
    final res = await send(request);
    final r = jsonDecode(utf8.decode(await res.stream.toBytes()));
    print(r);
    return '$contentUrl/${r[file.name]}';
  }
}
