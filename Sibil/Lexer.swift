//
//  Lexer.swift
//  Sibil
//
//  Created by Eryn Wells on 12/16/16.
//  Copyright © 2016 Eryn Wells. All rights reserved.
//

import Foundation


struct Token: CustomDebugStringConvertible {
    enum Kind {
        case LeftParen
        case RightParen
    }
    
    let kind: Kind
    let value: String

    // MARK: CustomDebugStringConvertible

    var debugDescription: String {
        return "Token(kind: .\(kind), value: \"\(value)\")"
    }
}


class Lexer {
    let input: String

    var index: String.Index

    init(input: String) {
        self.input = input
        self.index = input.startIndex
    }
}

extension Lexer: Sequence, IteratorProtocol {
    typealias Element = Token

    func makeIterator() -> Lexer {
        return self
    }

    func next() -> Token? {
        guard index != input.endIndex else {
            return nil
        }

        var token: Token?
        while token == nil {
            let c = input[index]
            switch c {
            case "(":
                token = Token(kind: .LeftParen, value: String(c))
            case ")":
                token = Token(kind: .RightParen, value: String(c))
            default:
                break
            }
            index = input.index(after: index)
        }
        return token
    }
}
