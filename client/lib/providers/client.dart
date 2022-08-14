import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:graphql/client.dart';
import 'package:notebook/models/user.dart';
import 'package:shared_preferences/shared_preferences.dart';

class Client extends ChangeNotifier {
  final SharedPreferences? sharedPrefs;

  static final Future<String> _schemaState =
      rootBundle.loadString('assets/api.graphql');
  static const String _sessionKey = "auth";
  static const url =
      String.fromEnvironment('API_URL', defaultValue: 'http://localhost');
  String? _session;

  Client(
    this.sharedPrefs,
  ) {
    _session = sharedPrefs?.getString(_sessionKey);
  }

  GraphQLClient get client {
    final httpLink = HttpLink(url);
    final wsLink = WebSocketLink(url);
    final transportLink =
        Link.split((request) => request.isSubscription, wsLink, httpLink);
    final authLink =
        AuthLink(getToken: () => _session == null ? null : 'Bearer $_session');
    final link = authLink.concat(transportLink);
    return GraphQLClient(link: link, cache: GraphQLCache());
  }

  String? get session => _session;

  set session(String? sess) {
    bool changed = sess == _session;
    _session = sess;
    if (changed) {
      notifyListeners();
    }
  }

  Future<User?> getUser() async {
    if (session == null) return null;
    return User.fromJson((await _request(operationName: 'users'))?['edges'][0]);
  }

  Future<String> signup(
      {required String name, required String password, String? email}) async {
    Map<String, dynamic> vars = {};
    vars['name'] = name;
    vars['password'] = password;
    vars['email'] = email;
    return await _request(operationName: 'signup', variables: vars);
  }

  Future<String> login(
      {required String identity, required String password}) async {
    return await _request(
        operationName: 'login',
        variables: {identity: identity, password: password});
  }

  Future<String> renewSession() async {
    return await _request(operationName: 'renewSession');
  }

  Future<dynamic> _request(
      {required String operationName,
      Map<String, dynamic> variables = const {},
      String? resultName}) async {
    final response = await client.query(QueryOptions(
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
}
