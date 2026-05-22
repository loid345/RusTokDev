import 'package:graphql/client.dart';

import 'client_config.dart';

class GraphQlClientFactory {
  const GraphQlClientFactory();

  GraphQLClient create(GraphQlClientConfig config) {
    final authLink = AuthLink(getToken: () async {
      final token = config.context.accessToken;
      if (token == null || token.isEmpty) {
        return null;
      }
      return 'Bearer $token';
    });

    final headers = Map<String, String>.from(config.headers)
      ..remove('Authorization');

    final httpLink = HttpLink(
      config.httpUri.toString(),
      defaultHeaders: headers,
    );

    final wsLink = WebSocketLink(
      config.wsUri.toString(),
      config: SocketClientConfig(
        initialPayload: () => config.wsInitPayload,
      ),
    );

    final transportLink = authLink.concat(
      Link.split((request) => request.isSubscription, wsLink, httpLink),
    );

    return GraphQLClient(
      cache: GraphQLCache(),
      link: transportLink,
      queryRequestTimeout: Duration(milliseconds: config.requestTimeoutMs),
    );
  }
}
